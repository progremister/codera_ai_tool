#Docker environment supports development in React, Angular, C#, and .NET,

FROM ubuntu:20.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Update and install common development tools and libraries
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    software-properties-common \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Node.js and npm for React and Angular
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g @angular/cli create-react-app

# .NET SDK for C# and .NET
RUN wget https://packages.microsoft.com/config/ubuntu/20.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && apt-get update \
    && apt-get install -y apt-transport-https \
    && apt-get update \
    && apt-get install -y dotnet-sdk-5.0

# Install Golang
RUN apt-get update && apt-get install -y golang-go \
  && rm -rf /var/lib/apt/lists/*

# Install OpenJDK
RUN apt-get update && apt-get install -y openjdk-11-jdk \
  && rm -rf /var/lib/apt/lists/*

# Install Python and package manager
RUN apt-get update && apt-get install -y python3.9 python3-pip \
  && rm -rf /var/lib/apt/lists/*

# Cleanup
RUN apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/man/?? /usr/share/man/??_*

WORKDIR /app

RUN mkdir /output

COPY . .

RUN cargo build --release

# The command to run your application, use actual application's binary name
CMD ["/app/target/release/codera"]