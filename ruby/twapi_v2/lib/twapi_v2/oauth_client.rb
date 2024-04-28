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

    def access_token(code, code_verifier)
      @client.authorization_code = code
      @client.access_token! code_verifier
    end

    def refresh(refresh_token)
      @client.refresh_token = refresh_token
      @client.access_token!
    end
  end
end
