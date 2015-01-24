require 'test_helper'

class PalletTest < ShpTest
  before do
    shp "start"
  end

  it "has nothing on the pallet and nothing that could be added yet" do
    output = shp "pallet"
    assert_match /Nothing has been loaded onto your current pallet yet!/, output
    assert_match /No changes that could be loaded./, output
  end

  describe "when you've created a new file" do
    let(:new_filename) { "some_file" }
    before do
      File.open(new_filename, "w") do |f|
        f.puts "A new file in my project"
      end
    end

    it "lists the new file as not on the pallet yet" do
      output = shp "pallet"
      expected_output = <<-HERE
Nothing has been loaded onto your current pallet yet!
You could load onto your pallet:
    #{new_filename}
HERE
      assert_match expected_output.strip, output
    end

    it "loads changes in the current directory onto the pallet" do
      skip "until load starts to be implemented"
      shp "load"

      output = shp "pallet"
      assert_match /Your current pallet contains:/, output
      assert_match /New files:/, output
      assert_match new_filename, output
      assert_match /No changes that could be loaded./, output
    end
  end
end