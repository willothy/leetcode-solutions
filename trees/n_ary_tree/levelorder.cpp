#include <vector>
using std::vector;

// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
public:
    vector<vector<int>> levelOrder(Node* root) {
        return *traverse(root, new vector<vector<int>>(), 0);
    }

    vector<vector<int>>* traverse(Node* root, vector<vector<int>>* res, int level) {
        if (root) {
            if (res->size() <= level) {
                res->push_back({});
            }
            (*res)[level].push_back(root->val);
            for (Node* child : root->children) {
                traverse(child, res, level + 1);
            }
        }
        return res;
    }
};