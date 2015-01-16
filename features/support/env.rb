require 'aruba/cucumber'

# Add rust's `target` directory to the PATH so we can just run the binary
ENV['PATH'] = "#{File.expand_path('target')}#{File::PATH_SEPARATOR}#{ENV['PATH']}"
