class Bibo < Formula
  desc "Fast, local neural text-to-speech CLI"
  homepage "https://larrykoo711.github.io/bibo"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-darwin-arm64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    end
    on_intel do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-darwin-x64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X64"
    end
  end

  on_linux do
    url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-linux-x64.tar.gz"
    sha256 "PLACEHOLDER_SHA256_LINUX"
  end

  depends_on "python@3.13"

  def install
    bin.install "bibo"

    # Create wrapper script that sets up Python environment
    (bin/"bibo").write <<~EOS
      #!/bin/bash
      export BIBO_PYTHON_DEPS_INSTALLED="${HOME}/.bibo/python-deps"

      # Install Python dependencies on first run
      if [ ! -f "$BIBO_PYTHON_DEPS_INSTALLED" ]; then
        echo "Installing Python dependencies (first run only)..."
        pip3 install --user piper-tts >/dev/null 2>&1
        mkdir -p "$(dirname "$BIBO_PYTHON_DEPS_INSTALLED")"
        touch "$BIBO_PYTHON_DEPS_INSTALLED"
      fi

      exec "#{libexec}/bibo" "$@"
    EOS

    libexec.install Dir["*"]
  end

  def caveats
    <<~EOS
      Bibo requires Python piper-tts for TTS synthesis.
      On first run, it will install piper-tts automatically.

      Quick start:
        bibo "Hello, world!"                 # Speak text
        bibo -l                              # List available voices
        bibo -d amy                          # Download a voice
        bibo "Hello" -v amy -o hello.wav     # Save to file
    EOS
  end

  test do
    assert_match "bibo", shell_output("#{bin}/bibo --version")
  end
end
