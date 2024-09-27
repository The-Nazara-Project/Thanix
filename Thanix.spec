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
Version:        0.1.0_beta.12
Release:        0.1
Summary:        Rust to yaml code generator
# FIXME: Select a correct license from https://github.com/openSUSE/spec-cleaner#spdx-licenses
License:        GPL-3.0
# FIXME: use correct group, see "https://en.opensuse.org/openSUSE:Package_group_guidelines"
# Group:
URL:            https://github.com/The-Nazara-Project/Thanix
Source0:        Thanix-%{version}.tar.gz
Source1:        vendor.tar.gz
Source2:        cargo_config
BuildRequires:  git
BuildRequires:  cargo
BuildRequires:  cargo-packaging

# the name of the actual binary
%define bin_name thanix

%description

Thanix is an experimental cli application written in Rust for generating Rust code from yaml schema files like they
are found in openAPI schemas.

%prep
%autosetup -p1 -a1
install -D -m 644 %{SOURCE2} .cargo/config

%build
%{cargo_build}

%install
# manually
install -D -d -m 0755 %{buildroot}%{_bindir}
install -m 0755 %{_builddir}/%{name}-%{version}/target/release/%{bin_name} %{buildroot}%{_bindir}/%{bin_name}


%files
%{_bindir}/%{bin_name}
%license LICENSE
%doc README.md

%changelog
* Fri Sep 27 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.12-0.1
- Scrub index arrays from serde_qs::to_string function from query strings
- Fix query struct fields not being public
* Fri Sep 06 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.9-0.1
- Fix type recognition logic
* Fri Sep 06 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.8-0.1
- Fix critical bug in workflow mode breaking type generation for sanitary structs
* Fri Sep 06 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.7-0.1
- Fix the version module not being public in output
* Fri Sep 06 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.6-0.1
- Generate output with feature to dynamically build a VERSION constant
* Thu Sep 05 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.5-0.1
- Add workaround to allow output to handle unsanitary response data
* Tue Feb 27 2024 Christopher Hock <christopher-hock@protonmail.com> - 0.1.0_beta.1-0.1
- Fully functional beta release
- Rewritten the path and type generation logic
- Handled API parameters of unspecified type
