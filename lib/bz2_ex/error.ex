defmodule Bz2Ex.Error do
  @moduledoc "Exception raised when compression or decompression fails."

  defexception [:reason, :operation]

  @type t :: %__MODULE__{reason: Bz2Ex.error_reason(), operation: :compress | :decompress}

  @impl true
  def message(%__MODULE__{reason: reason, operation: operation}) do
    "bzip2 #{operation} failed: #{format_reason(reason)}"
  end

  defp format_reason(:param_error), do: "invalid parameters"
  defp format_reason(:mem_error), do: "memory allocation failed"
  defp format_reason(:data_error), do: "data integrity error"
  defp format_reason(:data_error_magic), do: "invalid bzip2 header"
  defp format_reason(:unexpected_eof), do: "unexpected end of data"
  defp format_reason(:outbuff_full), do: "output buffer full"
  defp format_reason(:config_error), do: "configuration error"
  defp format_reason(:sequence_error), do: "invalid operation sequence"
  defp format_reason(:io_error), do: "I/O error"
  defp format_reason(reason), do: inspect(reason)
end
