require 'erb'
require 'camelizable'

class String
  include Camelizable
end

class_name = "PollFields"
src = "duration_minutes, end_datetime, id, options, voting_status"
ary = src.split(", ")
all_flag = true
erb = ERB.new(File.read("expantions.erb"))
puts erb.result(binding)