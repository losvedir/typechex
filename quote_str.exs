IO.read(:stdio, :all) 
|> Code.string_to_quoted!() 
|> IO.inspect(limit: :infinity, printable_limit: :infinity, charlists: :as_lists)
