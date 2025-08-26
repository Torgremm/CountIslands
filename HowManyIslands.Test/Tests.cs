using Xunit;
using HowManyIslandsLib;

namespace HowManyIslandsLib.Tests
{
    public class SolutionTests
    {
        [Fact]
        public void SingleIsland_AllConnected()
        {
            int[][] grid = new int[][]
            {
                new int[] {1,1,1,1,0},
                new int[] {1,1,0,1,0},
                new int[] {1,1,0,0,0},
                new int[] {0,0,0,0,0}
            };

            var sol = new HowManyIslandsLib.Solution(grid);
            Assert.Equal(1, sol.NumIslands());
        }

        [Fact]
        public void MultipleIslands_Separated()
        {
            int[][] grid = new int[][]
            {
                new int[] {1,1,0,0,0},
                new int[] {1,1,0,0,0},
                new int[] {0,0,1,0,0},
                new int[] {0,0,0,1,1}
            };

            var sol = new HowManyIslandsLib.Solution(grid);
            Assert.Equal(3, sol.NumIslands());
        }

        [Fact]
        public void EmptyGrid_NoIslands()
        {
            int[][] grid = new int[][]
            {
                new int[] {0,0,0},
                new int[] {0,0,0},
                new int[] {0,0,0}
            };

            var sol = new HowManyIslandsLib.Solution(grid);
            Assert.Equal(0, sol.NumIslands());
        }

        [Fact]
        public void CheckerboardPattern_EachOneSeparate()
        {
            int[][] grid = new int[][]
            {
                new int[] {1,0,1},
                new int[] {0,1,0},
                new int[] {1,0,1}
            };

            var sol = new HowManyIslandsLib.Solution(grid);
            Assert.Equal(5, sol.NumIslands());
        }

        [Fact]
        public void WonkyIslandTest()
        {
            int[][] grid = new int[][]
            {
                new int[] {1,0,1,1,0,0,1,0},
                new int[] {1,0,1,0,0,1,1,0},
                new int[] {0,0,0,0,1,0,0,1},
                new int[] {1,1,0,0,1,0,1,1},
                new int[] {0,1,0,1,0,0,0,0},
                new int[] {1,0,1,1,0,1,1,0}
            };

            var sol = new Solution(grid);
            int result = sol.NumIslands();

            Assert.Equal(9, result);
        }

        [Fact]
        public void LargeWonkyIslandTest()
        {
            int[][] grid = new int[][]
            {
                new int[] {1,0,1,1,0,0,1,0},
                new int[] {1,0,1,0,0,1,1,1},
                new int[] {1,0,0,0,1,0,0,1},
                new int[] {1,1,0,0,1,0,1,1},
                new int[] {0,1,0,1,0,0,1,0},
                new int[] {1,1,1,1,1,1,1,0}
            };

            var sol = new Solution(grid);
            int result = sol.NumIslands();

            Assert.Equal(3, result);
        }
    }
}
