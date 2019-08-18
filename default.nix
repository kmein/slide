{ stdenv, rustPlatform }:
rustPlatform.buildRustPackage rec {
  name = "slide-${version}";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "0s0x64d5l58nfhzg7j1zjbrf03l11v9qy5lqyrh0qfab0ba6mngb";

  meta = with stdenv.lib; {
    description = "Generate sliding windows of words";
    homepage = https://github.com/kmein/slide;
    license = licenses.mit;
    maintainers = [ maintainers.kmein ];
    platforms = platforms.all;
  };
}
