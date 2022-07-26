#**********************************
# Base layer, install dependencies*
#**********************************
FROM archlinux:latest as base
RUN pacman -Sy gcc --noconfirm;
RUN pacman -Sy clang --noconfirm;
RUN pacman -Sy make --noconfirm;
RUN pacman -Sy extra/cmake --noconfirm;
RUN pacman -Sy git --noconfirm;
RUN pacman -Sy base-devel --noconfirm;
RUN pacman -Sy extra/postgresql-libs --noconfirm;
RUN pacman -Sy wget --noconfirm;
FROM base as developer
ARG USERNAME=developer
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN pacman -Sy sudo --noconfirm;
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # add sudo support
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
 
USER $USERNAME
WORKDIR /workspace
RUN mkdir -p /home/developer/
RUN sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN /home/developer/.cargo/bin/cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8536
CMD ["sleep", "infinity"]
#**************************************
# Final layer, build example projects *
#**************************************