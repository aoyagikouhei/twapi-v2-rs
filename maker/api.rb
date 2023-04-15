require 'active_support'
require 'active_support/core_ext'
require 'erb'
require 'camelizable'
require 'yaml'

class String
  include Camelizable
end

yml = YAML.load_file(File.join('api', 'get_2_tweets_search_recent.yaml')).deep_symbolize_keys
fields = yml[:params].filter{|it| /.fields$/ =~ it[:name] }.map{|it| "#{it[:name].gsub(/\./, "_")}"}


erb = ERB.new(File.read("api.erb"))
puts erb.result(binding)