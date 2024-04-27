# frozen_string_literal: true

RSpec.describe TwapiV2 do
  it "has a version number" do
    expect(TwapiV2::VERSION).not_to be nil
  end

  it "scope" do
    expect(TwapiV2::Scope::TweetRead).to eq(:"tweet.read")
  end
end
