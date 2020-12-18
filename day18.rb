#!/usr/bin/env ruby

x = "(#{File.read("inputs/day18.in").strip.gsub("\n", ")+(")})"

class Integer
  def -(other)
    self * other
  end
end

puts eval(x.gsub("*", "-"))

class Integer
  def -(other)
    self * other
  end

  def /(other)
    self + other
  end
end

puts eval(x.gsub("*", "-").gsub("+", "/"))
