{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:

rustPlatform.buildRustPackage rec {
  pname = "pono";
  version = "0.2.0";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "v${version}";
    hash = "sha256-g3NZ9jf8gZfrhQfHsf+JW5fxB6FgsbGipMasHR1sMyM=";
  };

  cargoHash = "sha256-qCkKVH1CVKxzhhs5MFbYoJhJ9hSS52H3vZPLHdk/CvM=";

  checkPhase = ''
    cargo test
  '';

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  meta = with lib; {
    description = "pono - pack and organize symlinks once";
    homepage = "https://github.com/cristianoliveira/pono";
    changelog = "https://github.com/cristianoliveira/pono/releases/tag/${src.rev}";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}

