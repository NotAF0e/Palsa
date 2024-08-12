# 🚧 **UNDER CONSTRUCTION** 🚧
## Hi! If you want an interface to preview ableton 11 projects, specifically their *als* (ableton live set) files, this may be for you!

# So how does it work?
## Preface: Structure of ableton projects
Ableton usually only has other directories (dirs) and *als* files stored in the root dir as shown below. **Palsa** creates the projects dir so that users have a place to put their projects for preview. This is not a great way to do it, instead the user should be able to choose the path to their projects dir so that no copying is needed.
```
projects
-------- project_dir_0
         ----- other_dir
         ----- other_files
         ----- als_0.als
         ----- als_1.als
-------- project_dir_1
         ----- other_dir
         ----- other_files
         ----- als_0.als
         ----- als_1.als
```
## Extracting
The first step of the **palsa** pipline is extracting. This involves extracting the [gzipped](https://www.gzip.org/) contents of the *als* file, in other words it involves decompressing the contents. The result of this step is the humongous and bloated xml (Extensible Markup Language) content.
## Parsing
Not much can be found online about the structure of the content for newer ableton versions, therefore I fed an example *als* into a large language model (specifically gemeni 1.5 with 1 million context) and asked it to retreive the attributes that are required for previewing files. This was tough as some of the attribute names where quite obscure.

- als_file
    - name
    - group(s)
        - id
        - name
        - color
    - track(s)
        - group_id
        - name
        - color
        - clip(s)
            - name
            - start
            - end
            - loop_data
                - start
                - end

In the future the time signature and tempo will also be parsed so that a grid can be displayed in the preview.
## Caching
Finally to speed up loading tens of projects together containing hundreads of *als* files caching takes place so that the cache can be quickly loaded avoiding extracting and parsing entirely. This speeds up load times drastically.
## Final tidbits
Parsing and loading cache of projects as well as their *als* files is all done in parralel which is just an amazing performance boost. Also, thanks for reading! 😊
