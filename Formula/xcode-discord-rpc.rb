class XcodeDiscordRpc < Formula
  desc "A simple Discord Rich Presence client for Xcode"
  homepage "https://github.com/izyumidev/xcode-discord-rpc"
  url "https://github.com/izyumidev/xcode-discord-rpc/releases/download/v0.1.0/xcode-discord-rpc.tar.gz"
  sha256 "86869c5b3163768b3bcb7961fdd100407b8a9258de7d73846a5cdfd2882835be"
  version "0.1.0"

  def install
    bin.install "xcode-discord-rpc"
  end
end
