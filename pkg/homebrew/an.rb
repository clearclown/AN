class An < Formula
  desc "AN (安装) - Unified Package Manager for Linux"
  homepage "https://github.com/clearclown/AN"
  url "https://github.com/clearclown/AN/archive/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT"
  head "https://github.com/clearclown/AN.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "AN", shell_output("#{bin}/an --version")
  end
end
