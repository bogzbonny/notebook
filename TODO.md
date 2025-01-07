
^^^^^^^^^^^^ DONE ^^^^^^^^^^^^^^

GENERAL

- transcription with https://github.com/floneum/floneum

- qe... seperate binary... quick entry 
   - maybe the default nb interface opens up the mini-cli by default
     - if you hit enter immediately from here it will open up the default
       nb space.... I guess that makes sense as it will just open up the
       guy with no tags.

- nb --add [tags/or index number] <text too add>
   - quick addendum to an idea
   - maybe should be interactive 
      - only add the text afterwords 

- SHARING of ideas. It would be really cool to have some form of networking of
  ideas among others. There are then 3 catagories: public, private, and shared.
  - public ideas would be available to everyone and require no encryption
    - host on IPFS?
  - private ideas would be encrypted and only available to the creator
  - shared ideas would be encrypted on a user by user basis with the other users
    public key (and your private key) and would be available to the other user
    when they are online.
  - p2p file sync via onion?
    - https://github.com/cmars/onionpipe-rs
    - https://github.com/syncthing/syncthing
    - https://codepal.ai/code-generator/query/ElSz6TZX/rust-code-onionshare-hosted-website

- BACKUPS should be left to the user but could be facilitated through various
  means. ProtonDrive 

- return responses for all commands
  - add-tag needs a response showing the new idea file
  - `nb wc` should say something when there is nothing to transcribe

- do not show consumed files when normal listing files (ex 'nb some,tags') 
- do not show consumed files with `lsf` (but allow for it with a flag)

- If I want to start storing more information in the nb structure which is
  purely tagged based, it would be cool to allow for a folder type instead of a
  file. Hence we could have a folder which follows the nb tag structure. 
   - if you select this folder, it would expand the folder contents in the file
     selector window, whereby you could then further select the file to open. 
   - at the top of the autoexpanded folder would be an option: 
       >>> Open Folder in File Browser

- proper text consumtion: 
  - `nb 123 consumes 234,235`  123 would consume 234 and 235 adding the text to
    the bottom on 123 idea file and also adding the consumed tags to the 123 idea (deduped)
  - `nb 123 consumes 234`  could consume just one file
  - when text is appended it should be in surrounded by:
   -----------------------------
   Consumed from: 234
   ... the actual text...
   -----------------------------
  - FLAGS 
    - `--prepend` - preappend the consumed text to the top of the consuming file
    - `--no-append` - do not append the consumed text to the consuming file
    - `--no-tags` - do not add the consumed tags to the consuming file

- RIGHTHAND PREVIEW WINDOW when you're in the quick search mode
   - so the selector is on the righthand side, and the preview is on the right
     in a box... only issue may be if a lefthand file name is very long... then should word
     wrap it.
   - press tab to switch between the left filenav and the preview window - then
     can use the arrow keys to scroll the preview window
- search by # of words 
- search by filetype 

- `nb import ./pathtofile` import a file 
  - --autotag
    - would be cool to be able to autotag even with doc, docx, pdf, etc.
  - --tags=tag1,tag2,tag3
  - --dry-run
  - --recursive  
    - for importing a directory as seperate files
    - folder names are added as tags
    - --recursive-no-folder-tags
      - do not add folder names as tags

- configuration for the program which opens each file type
   
- MOVE THE CONFIG FILE TO THE NOTEBOOK DIRECTORY
 - have nb automatically assume the NOTEBOOK directory is ~/Desktop/NOTEBOOK
   or ~/NOTEBOOK

- LIST queried ideas in some form of preferential ordering
  - ordering options:
     - number of accesses - should this be in the file header?
     - edit date
     - creation date 
     - last access date
  - add the ordering options to the config file
  - would need to introduce some new persistent file structure to keep track of
    this information 
     - we should keep this persistent file in the NOTEBOOK directory

- add dynamic linking within the idea files like logseq has, this should:
  - <nb-1892> - this will link to idea number 1892 
  - when the cursor is over <1892>, need to build vim scripts so that `gf` will 
    open up that idea in a new tab


----------------------------------------------------------------------------------                                                                          
TRANSCRIPTION
- :q during transcription of tagging
- ability to use SHIFT+Enter during transcription
- UNDO command during transcription
- UNDO command during tagging

----------------------------------------------------------------------------------                                                                          
BOOKMARK MANAGER

- bookmark commands, where words on the same line as the link are considered to be the tags
  - Each bookmark should just be its own idea file (tags are normal) - should
    use a title=... tag for the title of the bookmark
  - WONT DO alternatively if there are no words on the line, grab the first line above
    (which is not seperated by an empty line) which also is not a line with a
    link in it as the line with tags. Basically allow for lists of links under
    a tag "title" line to still be searched. 
    - ability to retrieve header and add it to the same line as a bookmark
- drain bookmarks from browser to idea files 
- command for listing and opening bookmarks CAN JUST BE NORMAL 
  ('nb bookmarks,some,tags')
  - if all there is in a file is a link, then the link should be opened in the
    browser as opposed to the file being opened

----------------------------------------------------------------------------------                                                                          
LOWER PRIORITY                                                                          
10. ask the user if they want to create the new NOTEBOOK directory (and where) if the notebook directory cannot be found. 
20. audio recording note transcription
     - important for using as a dream catalog
        - keywords parsed out of the audio such as "add tag <tag>" "add tags <tag1><tag2>[and]<tag3>"
30. Suggested auto tagging
      - based on the content (word root prefixes), and all the existing tags which are already used.
        - would need a word root prefix getter. (common with oink project)
50. view trash
50. recover from trash
50. OCR text recognition
50. allow for a file ~/.notebook_location to be used to specify the location of the NOTEBOOK directory

