source_files = Rake::FileList.new("*.rs")

task :default => :binaries
task :binaries => source_files.ext('.bin')

rule ".bin" => ".rs" do |t|
  sh "rustc -o #{t.name} #{t.source}"
end
