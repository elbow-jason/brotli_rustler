defmodule BrotliRustler do
  @moduledoc """
  Fast brotli compression and decompression NIF rust via rustler.
  """
  use Rustler, otp_app: :brotli_rustler, crate: "brotli_rustler"

  @type compression_opt ::
          {:compression_level, non_neg_integer()}
          | {:lg_window_size, non_neg_integer()}
  @type compression_opts :: [compression_opt()]

  @doc """
  Compresses the given binary using the brotli algorithm.

  ## Examples

    iex> compress("hello world")
    {:ok, <<27, 10, 0, 0, 36, 64, 106, 144, 69, 106, 242, 156, 46>>}

    iex> opts = [compression_level: 10, lg_window_size: 5]
    iex> compress("hello", opts)
    {:ok, <<33, 16, 0, 4, 104, 101, 108, 108, 111, 3>>}
  """
  @spec compress(binary, compression_opts) :: {:ok, binary()} | {:error, String.t()}
  def compress(bin, opts \\ []) do
    compress_native(bin, opts[:compression_level] || 5, opts[:lg_window_size] || 22)
  end

  @doc false
  def compress_native(_bin, _compression_level, _lg_window_size) do
    :erlang.nif_error(:nif_not_loaded)
  end

  @doc """
  Decompress the given binary using the brotli algorithm.

  ## Examples

      iex> decompress(<<139, 2, 128, 98, 114, 111, 116, 108, 105, 3>>)
      {:ok, "brotli"}

  """
  @spec decompress(binary) :: {:ok, binary()} | {:error, String.t()}
  def decompress(_bin) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
