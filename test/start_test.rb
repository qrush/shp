require 'test_helper'

class StartTest < ShpTest
  it "starts with no ports" do
    shp "start"
    output = shp "ports"
    assert_match /No other ports have been added yet!/, output
    assert ! $?.success?
  end

  it "can start with a name" do
    shp "start foo"

    Dir.chdir("foo") do
      output = shp "ports"
      assert_match /No other ports have been added yet!/, output
      assert ! $?.success?
    end
  end

  it "can start with a URL" do
    shp "start https://github.com/qrush/m.git"

    Dir.chdir("m") do
      output = shp "ports"
      assert_equal "origin", output
      assert $?.success?
    end
  end
end
