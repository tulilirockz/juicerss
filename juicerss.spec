# Generated by rust2rpm 26
%bcond_without check

# Exclude input files from mangling
%global __brp_mangle_shebangs_exclude_from ^/usr/src/.*$

%global cargo_install_lib 0

%global crate juicerss

Name:           juicerss
Version:        1.1.0
Release:        %autorelease
Summary:        Read-only RSS feed reader

License:        BSD-3-Clause
URL:            https://crates.io/crates/juicerss
Source:         %{crates_source}
Source:         https://github.com/tulilirockz/juicerss/releases/download/v%{version}/vendor-v%{version}.tar.gz

BuildRequires:  openssl-devel
BuildRequires:  cargo-rpm-macros >= 26

%global _description %{expand:
Read-only RSS feed reader.}

%description %{_description}

Summary:        %{summary}
License:        BSD-3-Clause

%files       -n %{crate}
%license LICENSE
%license LICENSE.dependencies
%license cargo-vendor.txt
%doc README.md
%{_bindir}/juicerss

%prep
%autosetup -n %{crate}-%{version} -p1 -a1
%cargo_prep -v vendor

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies
%{cargo_vendor_manifest}

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
%autochangelog
