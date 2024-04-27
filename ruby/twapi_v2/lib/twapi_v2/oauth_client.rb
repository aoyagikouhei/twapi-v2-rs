# frozen_string_literal: true

require 'twitter_oauth2'

module TwapiV2
  class OauthClient
    def initialize(client_id, client_secret, redirect_uri)
      @client = TwitterOAuth2::Client.new(
        identifier: client_id,
        secret: client_secret,
        redirect_uri: redirect_uri
      )
    end
  end
end
