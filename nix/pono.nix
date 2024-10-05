{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:

rustPlatform.buildRustPackage rec {
  pname = "pono";
  version = "v0.3.0";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "${version}";
    hash = "sha256-PYRJwpi+RCEUixDN7/FP0WQdlaFa67QHn3u4qRahLq4=";
  };

  cargoHash = "sha256-BQ7TYVtU87fGPoLHQx2Ouc97BiGT4KyzKzlvNE9e2Jg=";

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

