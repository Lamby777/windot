# Maintainer: Cherry <arch@sparklet.org>
_pkgname=windot
pkgname=$_pkgname-git
pkgver=0.2.1.r85.ga3d8bf4
pkgrel=1
arch=(x86_64)
url="https://github.com/Lamby777/windot"
source=("$_pkgname::git+https://github.com/Lamby777/windot.git")
pkgdesc="A simple emoji picker."
md5sums=('SKIP')

depends=('gcc-libs' 'glibc' 'gcc' 'gtk4' 'pkgconf' 'libadwaita')
makedepends=(cargo git)

# Fetch the current version using the latest commit hash
pkgver() {
    cd "$srcdir/$_pkgname"
    # Extract the version from Cargo.toml using grep and sed
    cargo_ver=$(grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
    
    # Combine the crate version with the latest git commit count and hash
    git_ver=$(git rev-list --count HEAD)
    git_hash=$(git rev-parse --short HEAD)
    
    echo "$cargo_ver.r$git_ver.g$git_hash"
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
