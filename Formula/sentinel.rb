# Homebrew Formula for Sentinel
# Built by Glincker (A GLINR Product)
#
# Installation:
#   brew install glincker/tap/sentinel
#
# Documentation:
#   https://glincker.com/sentinel

class Sentinel < Formula
  desc "Process manager & system monitor for developers"
  homepage "https://glincker.com/sentinel"
  url "https://github.com/glincker/sentinel/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256_OF_TARBALL"
  license "MIT"
  head "https://github.com/glincker/sentinel.git", branch: "main"

  depends_on "rust" => :build
  depends_on "node" => :build

  def install
    # Build frontend
    system "npm", "install"
    system "npm", "run", "build"

    # Build Rust binary
    cd "src-tauri" do
      system "cargo", "build", "--release"
      bin.install "target/release/sentinel"
    end

    # Install man page (when available)
    # man1.install "docs/sentinel.1"

    # Install shell completions (when available)
    # bash_completion.install "completions/sentinel.bash"
    # zsh_completion.install "completions/_sentinel"
    # fish_completion.install "completions/sentinel.fish"
  end

  def caveats
    <<~EOS
      🛡️  Sentinel installed successfully!

      Get started:
        sentinel init              # Create config
        sentinel start             # Start processes
        sentinel gui               # Open desktop app

      Documentation:
        https://docs.glincker.com/sentinel

      Examples:
        $(brew --prefix)/share/sentinel/examples/

      Report issues:
        https://github.com/glincker/sentinel/issues

      Built with ❤️ by Glincker (A GLINR Product)
    EOS
  end

  test do
    # Test binary exists and runs
    assert_match version.to_s, shell_output("#{bin}/sentinel --version")

    # Test init command
    system bin/"sentinel", "init", "--help"

    # Test config validation (create temp config)
    (testpath/"sentinel.yaml").write <<~EOS
      processes:
        - name: test
          command: echo
          args:
            - "hello"
    EOS

    # Validate config loads without errors
    # (Full process testing requires daemon, skip for now)
  end
end
