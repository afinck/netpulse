Name: netpulse
Version: 0.1.0
Release: 1%{?dist}
Summary: A backend application for handling measurements and data storage with a web dashboard
License: MIT
Source0: %{name}-%{version}.tar.gz

BuildRequires: rust
BuildRequires: sqlite3-devel
BuildRequires: libprintpdf-dev
BuildRequires: axum
BuildRequires: chart.js

%description
Netpulse is a backend application that handles measurement and data storage using SQLite. It serves a web dashboard with interactive graphs and supports PDF export functionality.

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}/usr/local/bin
cp target/release/netpulse %{buildroot}/usr/local/bin/

mkdir -p %{buildroot}/usr/share/netpulse
cp -r static %{buildroot}/usr/share/netpulse/
cp -r templates %{buildroot}/usr/share/netpulse/
cp -r migrations %{buildroot}/usr/share/netpulse/

%files
/usr/local/bin/netpulse
/usr/share/netpulse/static
/usr/share/netpulse/templates
/usr/share/netpulse/migrations

%changelog
* Thu Oct 12 2023 Your Name <youremail@example.com> - 0.1.0-1
- Initial RPM package creation.