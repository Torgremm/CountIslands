namespace HowManyIslandsLib;

public class Solution
{

    private static int[][]? _grid = null;
    private static int _rows => _grid?.Length ?? 0;
    private static int _cols => _grid != null && _grid.Length > 0 ? _grid[0].Length : 0;

    public Solution(int[][] grid)
    {
        _grid = grid;
    }

    public int NumIslands()
    {
        if (_grid == null || _grid.Length == 0) return 0;
    
        int islands = 0;
    
        int[][] directions = new int[][] {new int[]{1,0}, new int[]{-1,0}, new int[]{0,1}, new int[]{0,-1}};
    
        for (int r = 0; r < _rows; r++)
        {
            for (int c = 0; c < _cols; c++)
            {
                if (_grid[r][c] == 0) //skip visited or water
                    continue;

                islands++;
                var queue = new Queue<(int, int)>();
                queue.Enqueue((r, c));
                _grid[r][c] = 0; //visited

                while (queue.Count > 0)
                {
                    var (x, y) = queue.Dequeue();
                    foreach (var d in directions)
                    {
                        int nx = x + d[0], ny = y + d[1]; //top left to bottom right traversal
                        if (nx >= 0 && nx < _rows && ny >= 0 && ny < _cols && _grid[nx][ny] == 1)
                        {
                            queue.Enqueue((nx, ny));
                            _grid[nx][ny] = 0; //visited
                        }
                    }
                }
            }
        }
    
        return islands;
    }

}
