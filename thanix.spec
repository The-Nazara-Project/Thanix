#
# spec file for package specRPM_CREATION_NAME
#
# Copyright (c) 2024 SUSE LLC
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via https://bugs.opensuse.org/
#


Name:           Thanix
Version:        v0.1.0-alpha.7
Release:        1.0
Summary:        Rust to yaml code generator.
# FIXME: Select a correct license from https://github.com/openSUSE/spec-cleaner#spdx-licenses
License:        GPL-3.0
# FIXME: use correct group, see "https://en.opensuse.org/openSUSE:Package_group_guidelines"
Group:          
URL:            https://github.com/The-Nazara-Project/Thanix
Source:         thanix-%{version}.tar.gz
BuildRequires:  git
BuildRequires:  cargo
BuildRequires:  cargo-packaging

# the name of the actual binary
%define bin_name thanix

%description

Thanix is an experimental cli application written in Rust for generating Rust code from yaml schema files like they
are found in openAPI schemas.

%prep
%autosetup

%build
%{cargo_build}

%install
# using cargo_install (only supports bindir)
# %{cargo_install}
# manually
install -D -d -m 0755 %{buildroot}%{_bindir}
install -m 0755 %{_builddir}/%{name}-%{version}/target/release/%{bin_name} %{buildroot}%{_bindir}/%{bin_name}

# useful if you want to run the program internal test suite
%{check}
%{cargo_test}

%post
%postun

%files
%{_bindir}/%{bin_name}
%license LICENSE
%doc README.md


