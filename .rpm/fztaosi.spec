%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: fztaosi
Summary: Old school intro
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: ASL 2.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

BuildRequires: upx
BuildRequires: libX11-devel
BuildRequires: libXi-devel
BuildRequires: mesa-libGL-devel
BuildRequires: alsa-lib-devel
Requires: libX11
Requires: libXi
Requires: mesa-libGL
Requires: alsa-lib

%description
%{summary}

%prep
%setup -q

%build
upx usr/bin/%{name}

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
