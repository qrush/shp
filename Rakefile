require 'rake/testtask'

task :default => [:build, :test]

desc "Build shp"
task :build do
  sh "cargo build"
end

Rake::TestTask.new do |t|
  t.libs << "test"
  t.test_files = FileList['test/*_test.rb']
end

desc "Install shp"
task :install do
  sh "cp target/shp /usr/local/bin/shp"
end
