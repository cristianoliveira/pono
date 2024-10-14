{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:
rustPlatform.buildRustPackage rec {
  pname = "pono";
  version = "master";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "${version}";
    hash = "sha256-h1tr4d3AO32EupsweIV2zGKqLDoASYTw9eVIXvMqpLU=";
  };

  cargoHash = "sha256-HolBfmAo/1HzLmVU7a8raC7X5ZXmqhZn7YE2CLnL1ns=";

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  meta = with lib; {
    description = "pono - pack and organize symlinks once";
    homepage = "https://github.com/cristianoliveira/pono/tree/develop";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}
