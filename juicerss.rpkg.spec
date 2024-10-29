%bcond_without check

%global cargo_install_lib 0

Name:           juicerss
Version:        1.0.1
Release:        %autorelease
Summary:        Read-only RSS feed reader

SourceLicense:  BSD-3-Clause
License:  BSD-3-Clause

URL:            https://github.com/tulilirockz/juicerss
VCS:			{{{ git_dir_vcs }}}
Source:			https://github.com/tulilirockz/juicerss/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 26

%global _description %{expand:
Read-only RSS feed reader.}

%description %{_description}

%prep
%autosetup -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/juicerss

%changelog
%autochangelog
