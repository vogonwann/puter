FROM rust:latest

# Install required development packages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpango1.0-dev \
    libgtk-4-dev \
    libadwaita-1-dev \
    gettext

WORKDIR /app
COPY . .

RUN cargo build --release

# Create directories and install files
RUN mkdir -p flatpak-build/bin && \
    mkdir -p flatpak-build/share/icons/hicolor/scalable/apps && \
    mkdir -p flatpak-build/share/applications && \
    mkdir -p flatpak-build/share/metainfo && \
    mkdir -p flatpak-build/share/locale/sr_RS/LC_MESSAGES && \
    cp target/release/puter flatpak-build/bin/ && \
    cp data/icons/hicolor/scalable/apps/lol.janjic.puter.svg flatpak-build/share/icons/hicolor/scalable/apps/ && \
    cp lol.janjic.puter.desktop flatpak-build/share/applications/ && \
    cp lol.janjic.puter.metainfo.xml flatpak-build/share/metainfo/ && \
    msgfmt -o puter.mo po/sr_RS/LC_MESSAGES/puter.po && \
    cp puter.mo flatpak-build/share/locale/sr_RS/LC_MESSAGES/
