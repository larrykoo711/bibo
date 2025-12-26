class Bibo < Formula
  desc "Fast, local neural text-to-speech CLI - zero dependencies"
  homepage "https://larrykoo711.github.io/bibo"
  version "0.3.0"
  license "MIT"

  # Sherpa-onnx TTS engine version
  SHERPA_VERSION = "1.12.20"

  on_macos do
    on_arm do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-darwin-arm64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_BIBO_ARM64"

      resource "sherpa" do
        url "https://github.com/k2-fsa/sherpa-onnx/releases/download/v#{SHERPA_VERSION}/sherpa-onnx-v#{SHERPA_VERSION}-osx-universal2-shared.tar.bz2"
        sha256 "PLACEHOLDER_SHA256_SHERPA_MACOS"
      end
    end

    on_intel do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-darwin-x64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_BIBO_X64"

      resource "sherpa" do
        url "https://github.com/k2-fsa/sherpa-onnx/releases/download/v#{SHERPA_VERSION}/sherpa-onnx-v#{SHERPA_VERSION}-osx-universal2-shared.tar.bz2"
        sha256 "PLACEHOLDER_SHA256_SHERPA_MACOS"
      end
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-linux-x64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_BIBO_LINUX_X64"

      resource "sherpa" do
        url "https://github.com/k2-fsa/sherpa-onnx/releases/download/v#{SHERPA_VERSION}/sherpa-onnx-v#{SHERPA_VERSION}-linux-x64-shared.tar.bz2"
        sha256 "PLACEHOLDER_SHA256_SHERPA_LINUX_X64"
      end
    end

    on_arm do
      url "https://github.com/larrykoo711/bibo/releases/download/v#{version}/bibo-linux-arm64.tar.gz"
      sha256 "PLACEHOLDER_SHA256_BIBO_LINUX_ARM64"

      resource "sherpa" do
        url "https://github.com/k2-fsa/sherpa-onnx/releases/download/v#{SHERPA_VERSION}/sherpa-onnx-v#{SHERPA_VERSION}-linux-aarch64-shared.tar.bz2"
        sha256 "PLACEHOLDER_SHA256_SHERPA_LINUX_ARM64"
      end
    end
  end

  def install
    # Install bibo binary
    bin.install "bibo"

    # Install bundled sherpa-onnx TTS engine
    resource("sherpa").stage do
      # Create sherpa directory structure
      (libexec/"sherpa").mkpath

      # Install bin and lib directories
      (libexec/"sherpa/bin").install Dir["bin/*"]
      (libexec/"sherpa/lib").install Dir["lib/*"]

      # Make binaries executable
      chmod 0755, Dir[libexec/"sherpa/bin/*"]
    end
  end

  def caveats
    <<~EOS
      Bibo is ready to use! No additional setup required.

      Quick start:
        bibo "Hello, world!"                 # Speak text (auto-downloads default voice)
        bibo "你好世界" -v melo              # Chinese + English bilingual voice
        bibo -l                              # List installed voices
        bibo -d list                         # Show downloadable voices
        bibo "Hello" -v amy -o hello.wav     # Save to file

      Sherpa-onnx TTS engine is bundled at: #{libexec}/sherpa

      Voice Models:
        melo    - Chinese + English bilingual (recommended)
        amy     - English (US)
        kss     - Korean
        huayan  - Chinese
    EOS
  end

  test do
    assert_match "bibo", shell_output("#{bin}/bibo --version")
  end
end
