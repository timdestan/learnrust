SOURCE = Rake::FileList.new("*.rs")
BINARIES = SOURCE.ext('').map do |file|
  "bin/#{file}"
end

task :default => :binaries
task :binaries => BINARIES

SOURCE.zip(BINARIES).each do |source, binary|
  rule binary => source do |t|
    sh "rustc -o #{t.name} #{t.source}"
  end
end

task :clean do
  rm_rf BINARIES
end
