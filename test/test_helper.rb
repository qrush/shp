require 'minitest/autorun'

class ShpTest < MiniTest::Test
  extend Minitest::Spec::DSL

  TEST_DIR = "./tmp/test"

  before do
    FileUtils.mkdir_p TEST_DIR
    @current_dir = Dir.pwd
    FileUtils.chdir TEST_DIR
  end

  after do
    FileUtils.chdir @current_dir
    FileUtils.rm_rf TEST_DIR
  end

  def shp(args)
    `#{File.expand_path(File.join(__FILE__, "..", "..", "target", "shp"))} #{args} 2>&1`.strip
  end
end
