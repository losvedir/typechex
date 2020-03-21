# Typechex

The goal of this project is to be an Elixir typechecker that's stronger than dialyzer. Main inspiration is the rust (which this project is written in) type checker.

## Use

`$ cargo run /path/to/elixir/project/lib`


## Comparison to dialyzer and other type systems

Dialyzer reading list:

* [Initial paper (pdf)](https://www.it.uu.se/research/group/hipe/dialyzer/publications/succ_types.pdf) - Introduces theoretical foundation of dialyzer, "success typings", which is essentially that dialyzer should never have false negatives.

# Limitations of dialyzer

## Success typing

Dialyzer uses "success typing" which avoids false positives. By default, it will only flag your code if it can determine that it will *always* fail. What this buys over 100% line test coverage I'm not sure. Consider these functions which dialyzer considers acceptable:

```ex
defmodule DialyzerTests do
  def foo(x) do
    case x do
      :a -> :c
      :b -> :d
    end
  end

  def bar(:c), do: :e

  def problematic(x) do
    x |> foo() |> bar()
  end
end
```

The problem is that `foo/1` can return either `:c` or `:d`, but `bar/1` can only handle `:c`, so `foo |> bar` is inappropriate. However, because `foo` could *happen to* always take the `:a -> :c` branch, then there *might* not be a bug here (just crufty, useless code).

For people who like static typing, this is clearly not as helpful as it could be (if it's even helpful at all, above thorough tests).

## Too-broad and too-narrow specs

Dialyzer does limited checking that your `@spec` contract is correct. For instance, it will flag this:

```ex
defmodule DialyzerTests do
  @spec foo(String.t(), String.t()) :: String.t()
  def foo(a, b), do: a + b
end

```

with this error:

```
lib/dialyzer_tests.ex:1:invalid_contract
The @spec for the function does not match the success typing of the function.

Function:
DialyzerTests.foo/2

Success typing:
@spec foo(number(), number()) :: number()
```

Dialyzer infers the "success typing" (what types the function handles and what types it returns) and compares it to the given spec. If they're totally different, it warns. The problem, is that if the spec is either too narrow or too broad, but has *some* overlap, it won't warn. Both of these pass default dialyzer checks:

```ex
defmodule DialyzerTests do
  @spec spec_too_broad(:a | :b) :: :z
  def spec_too_broad(:a), do: :z

  @spec spec_too_narrow(:a) :: :z
  def spec_too_narrow(:a), do: :z
  def spec_too_broad(:b), do: :z
end
```

## overspecs/underspecs, so close yet so far

Dialyzer has two flags, `:overspecs` and `:underspecs`, which almost help the above too-broad / too-narrow situation, but are unfortunately wrong. You can enable them in your mix project config, with `dialyzer: [flags: [:overspecs, :underspecs]]`.

Those warn if the success typing is too strict or too lax, respectively. For example, if dialyzer determines that the success typing is `:a | :b` but your spec says the function only takes `:a`, then `:overspecs` will cause a warning because it's "overspecified" or too strict. If your spec says `:a | :b | :c` then `:underspecs` warns because it's underspecified or too lax.

This is *so close* to what static typing aficionados want! The problem is that "underspecified" and "overspecified" are the *opposite* when talking about output, but dialyzer treats them the same as above.

Consider what will actually cause code to break. If your function handles *less* than what your spec says, then callers abiding by the spec can break if they call it with the wrong thing. If your function returns *more* than what your spec says, then callers abiding by the spec can break if they get the wrong return value.

So to get reasonable type coverage, you have to enable both `:underspecs` and `:overspecs`. But because dialyzer has it backwards for output values, `:underspecs` warns on totally valid code. For instance:

```ex
defmodule DialyzerTests do
  @type color :: :red | :orange | :yellow | :green | :blue | :indigo | :violet

  @spec favorite_color(String.t()) :: color()
  def favorite_color(name) do
    case name do
      "John" -> :red
      "Jane" -> :orange
      _ -> :yellow
    end
  end
end
```

Perhaps you have an enum type, used throughout your app, like `color()` above. If some function happens to only return some of the variants, that's totally fine. You still want to type it as returning `color()` since in the future it might return other variants. However, dialyzer with `:underspecs` warns on this function, because it considers the spec "too lax".

Conversely, `:overspecs` will warn on valid code. Consider:

```ex
defmodule DialyzerTests do
  @spec foo(integer(), integer()) :: integer()
  def foo(a, b) do
    a + b
  end
end
```

Dialyzer determines the success typing to be `foo(number(), number()) :: number()`. The code indeed handles floats, inadvertently, because `Kernel.+/2` happens to work with floats. But perhaps in your code's domain you only *intend* for this function to be used with integers. Even so, `:overspecs` will warn and prevent you from annotating it that way. (In this case, you could add `when is_integer(a) and is_integer(b)` as a guard, but that grows tedious and verbose and *should* be unnecessary.)

This issue was [raised on the mailing list](http://erlang.org/pipermail/erlang-questions/2017-March/091968.html) but it doesn't seem like there are any plans to fix it. Formally, this is related to [covariance and contravariance](https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)) and how subtypes should be contravariant on the input and covariant on the output.
