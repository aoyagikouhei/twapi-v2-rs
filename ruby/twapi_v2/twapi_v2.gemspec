# frozen_string_literal: true

require_relative "lib/twapi_v2/version"

Gem::Specification.new do |spec|
  spec.name = "twapi_v2"
  spec.version = TwapiV2::VERSION
  spec.authors = ["Kouhei Aoyagi"]
  spec.email = ["aoyagi.kouhei@gmail.com"]

  spec.summary = "Twitter V2 library"
  spec.description = "Twitter V2 library"
  spec.homepage = "https://github.com/aoyagikouhei/twapi-v2-rs"
  spec.required_ruby_version = ">= 3.2.0"

  spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/aoyagikouhei/twapi-v2-rs"
  spec.metadata["changelog_uri"] = "https://github.com/aoyagikouhei/twapi-v2-rs/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  spec.add_runtime_dependency 'twitter_oauth2', '~> 0.5.1'
  spec.add_runtime_dependency  'x', '~> 0.14.1'

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
