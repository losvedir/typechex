case System.argv() do
  [path] -> 
    case Mix.Utils.extract_files([path], ["ex"]) do
      [] ->
        IO.puts(:stderr, "No files found at path #{inspect(path)}")

      paths ->
        Enum.each(paths, fn file ->
          IO.puts("@!&*(^)|#{inspect(file)}|)&@^#%")
          
          file
          |> File.read!() 
          |> Code.string_to_quoted!() 
          |> IO.inspect(limit: :infinity, printable_limit: :infinity, charlists: :as_lists)
        end)
    end

  other -> 
    IO.puts(:stderr, "Expected one argument, a path, got #{inspect(other)}")
    ""
end
