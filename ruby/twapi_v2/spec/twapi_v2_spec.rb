# frozen_string_literal: true

RSpec.describe TwapiV2 do
  it "has a version number" do
    expect(TwapiV2::VERSION).not_to be nil
  end

  it "scope" do
    expect(TwapiV2::Scope::TweetRead).to eq(:"tweet.read")
  end

  it "scopes" do
    expect(TwapiV2::Scopes::FollowCheck).to eq([:"users.read", :"offline.access"])
  end

  it "client new" do
    expect(TwapiV2::OauthClient::new("a", "b", "c")).not_to be nil
  end

  it "oauth url" do
    client = TwapiV2::OauthClient::new("a", "b", "c")
    url, pkce, state = client.oauth_url(TwapiV2::Scopes::FollowCheck)
    expect(url).not_to be nil
    expect(pkce).not_to be nil
    expect(state).not_to be nil
  end
end
