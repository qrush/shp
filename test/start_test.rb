require 'test_helper'

class StartTest < ShpTest
  it "starts with no ports" do
    shp "start"
    output = shp "ports"
    assert_equal "Argh! No other ports be known to us yet!", output
  end

  it "can start with a name" do
    shp "start foo"

    Dir.chdir("foo") do
      output = shp "ports"
      assert_equal "Argh! No other ports be known to us yet!", output
    end
  end

  it "can start with a URL" do
    shp "start https://github.com/qrush/m.git"

    Dir.chdir("m") do
      output = shp "ports"
      assert_equal "origin", output
    end
  end
end
