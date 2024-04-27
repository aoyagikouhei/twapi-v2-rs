# frozen_string_literal: true

require_relative "twapi_v2/oauth_client"
require_relative "twapi_v2/scope"
require_relative "twapi_v2/version"

module TwapiV2
  class Error < StandardError; end
end
