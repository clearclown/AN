class An < Formula
  desc "AN (安装) - Unified Package Manager for Linux"
  homepage "https://github.com/clearclown/AN"
  url "https://github.com/clearclown/AN/archive/v0.1.1.tar.gz"
  sha256 "" # Update with: shasum -a 256 an-0.1.1.tar.gz
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/an", "--version"
  end
end
