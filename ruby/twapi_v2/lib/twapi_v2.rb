# frozen_string_literal: true

require_relative "twapi_v2/version"
require_relative "twapi_v2/scope"

module TwapiV2
  class Error < StandardError; end
  # Your code goes here...

  def self.hello
    "Hello World"
  end
end
