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

    def oauth_url(scopes)
      authorization_uri = @client.authorization_uri(scope: scopes)
      [authorization_uri, @client.code_verifier, @client.state]
    end
  end
end