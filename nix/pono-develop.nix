{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:
rustPlatform.buildRustPackage rec {
  pname = "pono";
  version = "develop";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "${version}";
    hash = "sha256-FhXSUc5XKrworPmBejPtPHi9/EYhzi4ZtMeWBOf27h8=";
  };

  cargoHash = "sha256-w41aMIMdlCoOVGfVfx0p7RPSP5URx39F95cB2H4cUjA=";

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
