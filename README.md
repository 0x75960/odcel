odcel
------

[one-demensional cellular automaton](http://mathworld.wolfram.com/ElementaryCellularAutomaton.html)

```
USAGE:
    odcel.exe <RULE> <GENERATION>

ARGS:
    <RULE>          rule number 0-255
    <GENERATION>    num of generation
```


### sample

#### rule 90 for 25 generations

```console
$ odcel 90 25
                         *
                        * *
                       *   *
                      * * * *
                     *       *
                    * *     * *
                   *   *   *   *
                  * * * * * * * *
                 *               *
                * *             * *
               *   *           *   *
              * * * *         * * * *
             *       *       *       *
            * *     * *     * *     * *
           *   *   *   *   *   *   *   *
          * * * * * * * * * * * * * * * *
         *                               *
        * *                             * *
       *   *                           *   *
      * * * *                         * * * *
     *       *                       *       *
    * *     * *                     * *     * *
   *   *   *   *                   *   *   *   *
  * * * * * * * *                 * * * * * * * *
 *               *               *               *
 ```

 #### rule 60 for 24 generations

 ```
 $ odcel 60 24
                        *
                        **
                        * *
                        ****
                        *   *
                        **  **
                        * * * *
                        ********
                        *       *
                        **      **
                        * *     * *
                        ****    ****
                        *   *   *   *
                        **  **  **  **
                        * * * * * * * *
                        ****************
                        *               *
                        **              **
                        * *             * *
                        ****            ****
                        *   *           *   *
                        **  **          **  **
                        * * * *         * * * *
                        ********        ********
```
