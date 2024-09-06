# Maintainer: Cherry <arch@sparklet.org>
_pkgname=windot
pkgname=$_pkgname-git
pkgver=0.2.1.58da11e
pkgrel=1
arch=(x86_64)
url="https://github.com/Lamby777/windot"
source=("$_pkgname::git+https://github.com/Lamby777/windot.git")
pkgdesc="A simple emoji picker."
md5sums=('SKIP')

depends=('gtk4')
makedepends=('gcc-libs' 'glibc' 'gcc' 'cargo' 'git' 'pkgconf' 'libadwaita')

# Fetch the current version using the latest commit hash
pkgver() {
    cd "$srcdir/$_pkgname"
    # get the crate version using grep and sed
    cargo_ver=$(grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
    
    # use the latest commit hash
    git_hash=$(git rev-parse --short HEAD)
    
    echo "$cargo_ver.$git_hash"
}

build() {
    cd "$srcdir/$_pkgname"
    make 
}

package() {
    cd "$srcdir/$_pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"

    make install DESTDIR="$pkgdir/" prefix="/usr"
}
