require 'test_helper'

class StartTest < ShpTest
  it "starts with no ports" do
    shp "start"
    output = shp "ports"
    assert_match /No other ports have been added yet!/, output
    assert ! $?.success?
  end

  it "starts with a name" do
    shp "start foo"

    Dir.chdir("foo") do
      output = shp "ports"
      assert_match /No other ports have been added yet!/, output
      assert ! $?.success?
    end
  end

  {
    http: "https://github.com/qrush/m.git",
    ssh: "git@github.com:qrush/m.git",
    git: "git://github.com/qrush/m.git"
  }.each do |protocol, url|
    it "starts with a #{protocol} URL" do
      shp "start #{url}"

      Dir.chdir("m") do
        output = shp "ports"
        assert_equal "origin", output
        assert $?.success?
      end
    end
  end
end
