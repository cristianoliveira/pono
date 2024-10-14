{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:
rustPlatform.buildRustPackage rec {
  pname = "pono";
  version = "main";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "pono";
    rev = "${version}";
    hash = "sha256-5PyiJ/n9n3CvDMOd7OVv1KK9uwWLLTnx9TfDzEDboX4=";
  };

  cargoHash = "sha256-RhrzKeLn5R68zQAYteGQF4hg4eqBNbF+Z9du39dZSfc=";

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
