{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:
rustPlatform.buildRustPackage rec {
  pname = "pono";
  ## Latest commit on main branch
  version = "efa714c7e44933b0904a1b6001745e1b664715cc";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "${version}";
    hash = "sha256-LrtGvXTiTOl7gJzfqzOkmZe8Nf7e71e65GW+P4jD/FM=";
  };

  cargoHash = "sha256-PkBDKuY5l9yHeGbA55l9rsspjBFx4zssIWxy7QAqDsg=";

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
