<h1 style="text-align: center;">HyperExplorer</h1>
<p style="text-align: justify;">HyperExplorer is a file explorer with really fast search times.</p>
<br />

![screenshot](screenshots/1.png)

## Performance
### Machine Specifications:
```
CPU: AMD Ryzen 5 8600G
RAM: 32GB DDR5 4800MT/s
Storage: 1TB NVMe SSD (6Gb/s write, 7.3Gb/s read) PCIE 4.0
Operating System: Kubuntu 25.04
```
Indexing the OS root directory takes ``3.3670`` seconds.
<br />
Searching takes ``162.68`` milliseconds.
<br />
You can test the performance running <pre>```cargo bench```</pre>

## Roadmap
- [x] Basic navigation
- [x] Opening files
- [ ] Context menu
- [ ] Drag and drop
- [x] File and directory icons
- [x] Fast Search
- [ ] Better UI

## Download
You can download the latest release from the [releases](https://github.com/triplean/hyperexplorer/releases) page.

## Building and running 
You can build the project with <pre>```cargo build```</pre> or build and run it directly with <pre>```cargo run```</pre>

## Contributing
I'd love to see contributions to this project!  
If you want to contribute, you can create an [issue](https://github.com/triplean/hyperexplorer/issues/new/choose) to report bugs or suggest features, or send a [pull request](https://docs.github.com/articles/about-pull-requests) directly.
