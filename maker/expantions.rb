require 'erb'
require 'camelizable'

class String
  include Camelizable
end

class_name = "PollFields"
src = "duration_minutes, end_datetime, id, options, voting_status"
ary = src.split(", ")
erb = ERB.new(File.read("expantions.erb"))
puts erb.result(binding)